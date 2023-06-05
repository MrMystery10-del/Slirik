package core;

import keywords.Else;
import keywords.If;
import keywords.While;
import statements.*;

import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;

public class Trees {

    // Constructor for type tree
    protected static List<Statement> typeTree(Iterator<Lexer.Token> tokens) {
        return typeTree(tokens, tokens.next());
    }

    // Constuructor of a type Tree that takes a stream of tokens, Starting from the given token (currentToken)
    protected static List<Statement> typeTree(Iterator<Lexer.Token> tokens, Lexer.Token currentToken) {
        List<Statement> statements = new LinkedList<>();

        Lexer.Token token = currentToken;
        statements.add(new Type(token.value()));
        // Checks the type of token 
        token = tokens.next();
        if (token.tokenType() == Lexer.TokenType.END) {
            return statements;
        } else if (token.tokenType() == Lexer.TokenType.IDENTIFIER) {
            // Calls indenfierTree and adds reslut to the statement list
            statements.addAll(identifierTree(tokens, token));\
            // Exeption if token is neither END or IDENTIFIER
        } else throw new IllegalArgumentException(token.value() + " is not allowed after type");

        return statements;
    }

    // Constructor for identifier tree from a stream of tokens
    protected static List<Statement> identifierTree(Iterator<Lexer.Token> tokens) {
        return identifierTree(tokens, tokens.next());
    }


     // Constructor for identifier tree from a stream of tokens, starting from a given token (currentToken)
    protected static List<Statement> identifierTree(Iterator<Lexer.Token> tokens, Lexer.Token currentToken) {
        List<Statement> statements = new LinkedList<>();

        Lexer.Token token = currentToken;
        statements.add(new Variable(token.value()));
        statements.add(new Load(token.value()));

        token = tokens.next();
        // Checks the token type
        if (token.tokenType() == Lexer.TokenType.EQUALS) {
            statements.addAll(equalsTree(tokens));
        } else if (token.tokenType() == Lexer.TokenType.BINARY_OPERATOR) {
            statements.addAll(equalsTree(tokens, false));
        } else if (token.tokenType() == Lexer.TokenType.END) {
            return statements;
            // Exception of token is not one of the above listed
        } else throw new IllegalArgumentException(token.value() + " is not allowed after identifier");

        return statements;
    }

    // Constructor for equals tree from a stream of tokens
    private static List<Statement> equalsTree(Iterator<Lexer.Token> tokens) {
        return equalsTree(tokens, true);
    }

    // Constructor for equals tree from a stream of tokens, optionally starting with a new value
    private static List<Statement> equalsTree(Iterator<Lexer.Token> tokens, boolean newValue) {
        List<Statement> statements = new LinkedList<>();

        if (newValue)
            statements.add(new Setter("0"));
        statements.add(new Operation("+"));

        Lexer.Token token;
        // While loop that iterates until there are no more tokens
        while (tokens.hasNext()) {
            token = tokens.next();
            
            // Checks the token type
            switch (token.tokenType()) {
                case NUMBER -> statements.add(new Adder(token.value()));
                case IDENTIFIER -> statements.add(new Getter(token.value()));
                case BINARY_OPERATOR -> statements.add(new Operation(token.value()));
            }
        }

        return statements;
    }

    // Creates a keyword tree from a stream of tokens
    protected static List<Statement> keywordTree(Iterator<Lexer.Token> tokens) {
        List<Statement> statements = new LinkedList<>();

        Lexer.Token token = tokens.next();
        String keyword = token.value();

        token = tokens.next();
        // Checks for if and while statements
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
            // Checks for while statements
            } else {
                statements.addAll(new While(headTokens).getKeywordBody());
                statements.addAll(bodyTree(true, tokens));
            }
            // Checks for else statements
        } else if (keyword.equals("else")) {
            statements.addAll(new Else().getKeywordBody());
            statements.addAll(bodyTree(false, tokens));
        }

        return statements;
    }

    // Creates a bopdy tree from a stream of tokens

    private static List<Statement> bodyTree(boolean jump, Iterator<Lexer.Token> tokens) {
        List<Statement> statements = new LinkedList<>();
        Lexer.Token token = tokens.next();

        List<Lexer.Token> expressionTokens = new LinkedList<>();

        boolean foundBody = false;
        int closedParensAvailable = 1;
        // Adds tokens if they're in parenthesis
        while (closedParensAvailable != 0) {
            expressionTokens.add(token);
            token = tokens.next();

            if (token.tokenType() == Lexer.TokenType.OPEN_PAREN)
                closedParensAvailable++;
            else if (token.tokenType() == Lexer.TokenType.CLOSE_PAREN)
                closedParensAvailable--;
        }
        statements.addAll(identifyToken(expressionTokens.iterator()));

        //Checks for Jump statements
        if (jump) statements.add(new Jump("NONE"));
        statements.add(new End());

        return statements;
    }

    // Identifier for type of token ofr the correct tree construction
    private static List<Statement> identifyToken(Iterator<Lexer.Token> tokens) {
        List<Statement> statements = new LinkedList<>();

        Lexer.Token token;
        // Iterates through tokens until there are no more
        while (tokens.hasNext()) {
            token = tokens.next();

            // Checks the token type
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
