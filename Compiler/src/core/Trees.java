package core;

import keywords.If;
import keywords.While;
import statements.*;

import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;

public class Trees {

    protected static List<Statement> typeTree(Iterator<Lexer.Token> tokens) {
        return typeTree(tokens, tokens.next());
    }

    protected static List<Statement> typeTree(Iterator<Lexer.Token> tokens, Lexer.Token currentToken) {
        List<Statement> statements = new LinkedList<>();

        Lexer.Token token = currentToken;
        statements.add(new Type(token.value()));

        token = tokens.next();
        if (token.tokenType() == Lexer.TokenType.END) {
            return statements;
        } else if (token.tokenType() == Lexer.TokenType.IDENTIFIER) {
            statements.addAll(identifierTree(tokens, token));
        } else throw new IllegalArgumentException(token.value() + " is not allowed after type");

        return statements;
    }

    protected static List<Statement> identifierTree(Iterator<Lexer.Token> tokens) {
        return identifierTree(tokens, tokens.next());
    }

    protected static List<Statement> identifierTree(Iterator<Lexer.Token> tokens, Lexer.Token currentToken) {
        List<Statement> statements = new LinkedList<>();

        Lexer.Token token = currentToken;
        statements.add(new Variable(token.value()));
        statements.add(new Load(token.value()));

        token = tokens.next();
        if (token.tokenType() == Lexer.TokenType.EQUALS) {
            statements.addAll(equalsTree(tokens));
        } else if (token.tokenType() == Lexer.TokenType.BINARY_OPERATOR) {
            statements.addAll(equalsTree(tokens, false));
        } else if (token.tokenType() == Lexer.TokenType.END) {
            return statements;
        } else throw new IllegalArgumentException(token.value() + " is not allowed after identifier");

        return statements;
    }

    private static List<Statement> equalsTree(Iterator<Lexer.Token> tokens) {
        return equalsTree(tokens, true);
    }

    private static List<Statement> equalsTree(Iterator<Lexer.Token> tokens, boolean newValue) {
        List<Statement> statements = new LinkedList<>();

        if (newValue)
            statements.add(new Setter("0"));
        statements.add(new Operation("+"));

        Lexer.Token token;
        while (tokens.hasNext()) {
            token = tokens.next();

            switch (token.tokenType()) {
                case NUMBER -> statements.add(new Adder(token.value()));
                case IDENTIFIER -> statements.add(new Getter(token.value()));
                case BINARY_OPERATOR -> statements.add(new Operation(token.value()));
            }
        }

        return statements;
    }

    protected static List<Statement> keywordTree(Iterator<Lexer.Token> tokens) {
        List<Statement> statements = new LinkedList<>();

        Lexer.Token token = tokens.next();
        String keyword = token.value();

        token = tokens.next();
        if (keyword.equals("if") || keyword.equals("while")) {
            List<Lexer.Token> headTokens = new LinkedList<>();

            // Add the tokens before the opening parenthesis to the headTokens list
            while (token.tokenType() != Lexer.TokenType.OPEN_PAREN) {
                headTokens.add(token);
                token = tokens.next();
            }

            if (keyword.equals("if")) {
                statements.addAll(new If(headTokens).getKeywordBody());
                statements.addAll(bodyTree(false, tokens));
            } else {
                statements.addAll(new While(headTokens).getKeywordBody());
                statements.addAll(bodyTree(true, tokens));
            }
        }

        return statements;
    }

    private static List<Statement> bodyTree(boolean jump, Iterator<Lexer.Token> tokens) {
        List<Statement> statements = new LinkedList<>();
        Lexer.Token token = tokens.next();

        List<Lexer.Token> expressionTokens = new LinkedList<>();

        boolean foundBody = false;
        int closedParensAvailable = 1;
        while (closedParensAvailable != 0) {
            expressionTokens.add(token);
            token = tokens.next();

            if (token.tokenType() == Lexer.TokenType.OPEN_PAREN)
                closedParensAvailable++;
            else if (token.tokenType() == Lexer.TokenType.CLOSE_PAREN)
                closedParensAvailable--;
        }
        statements.addAll(identifyToken(expressionTokens.iterator()));

        if (jump) statements.add(new Jump("NONE"));
        statements.add(new End());

        return statements;
    }

    private static List<Statement> identifyToken(Iterator<Lexer.Token> tokens) {
        List<Statement> statements = new LinkedList<>();

        Lexer.Token token;
        while (tokens.hasNext()) {
            token = tokens.next();

            statements.addAll(switch (token.tokenType()) {
                case TYPE -> typeTree(tokens, token);
                case IDENTIFIER -> identifierTree(tokens, token);
                case KEYWORD -> keywordTree(tokens);
                default -> throw new IllegalArgumentException("Illegal start of statement");
            });
        }
        return statements;
    }
}
