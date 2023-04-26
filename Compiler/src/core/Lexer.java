package core;

import java.util.ArrayList;
import java.util.List;

public class Lexer {

    public enum TokenType {
        TYPE, IDENTIFIER, EQUALS, NUMBER, BINARY_OPERATOR, OPEN_PAREN, CLOSE_PAREN, END,
        KEYWORD, FUNCTION, CONDITION
    }

    protected static List<Token> tokenize(String s) {
        List<Token> tokenList = new ArrayList<>();
        int i = 0;
        while (i < s.length()) {
            char c = s.charAt(i);
            if (Character.isWhitespace(c)) {
                i++;
                continue;
            }
            if (Character.isDigit(c) || c == '.') {
                StringBuilder numberBuilder = new StringBuilder();
                while (i < s.length() && Character.isDigit(s.charAt(i)) || s.charAt(i) == '.') {
                    numberBuilder.append(s.charAt(i));
                    i++;
                }
                tokenList.add(new Token(numberBuilder.toString(), TokenType.NUMBER));
                continue;
            }
            if (Character.isLetter(c)) {
                StringBuilder identifierBuilder = new StringBuilder();
                while (i < s.length() && Character.isLetterOrDigit(s.charAt(i))) {
                    identifierBuilder.append(s.charAt(i));
                    i++;
                }
                String identifier = identifierBuilder.toString();
                if (identifier.equals("int") || identifier.equals("float") || identifier.equals("bool")) {
                    tokenList.add(new Token(identifier, TokenType.TYPE));
                } else if (identifier.equals("if") || identifier.equals("else") || identifier.equals("while") || identifier.equals("for")) {
                    tokenList.add(new Token(identifier, TokenType.KEYWORD));
                } else {
                    tokenList.add(new Token(identifier, TokenType.IDENTIFIER));
                }
                continue;
            }
            if (c == '=') {
                tokenList.add(new Token(String.valueOf(c), TokenType.EQUALS));
                i++;
                continue;
            }
            if (c == '{') {
                tokenList.add(new Token(String.valueOf(c), TokenType.OPEN_PAREN));
                i++;
                continue;
            }
            if (c == '}') {
                tokenList.add(new Token(String.valueOf(c), TokenType.CLOSE_PAREN));
                i++;
                continue;
            }
            if (c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '^' || c == '#') {
                tokenList.add(new Token(String.valueOf(c), TokenType.BINARY_OPERATOR));
                i++;
                continue;
            }
            if (c == '<' || c == '>'){
                tokenList.add(new Token(String.valueOf(c), TokenType.CONDITION));
                i++;
                continue;
            }
            if (c == ';') {
                tokenList.add(new Token(";", TokenType.END));
                break;
            }
            throw new IllegalArgumentException("Invalid character: " + c);
        }
        return tokenList;
    }

    public record Token(String value, TokenType tokenType) {

        @Override
        public String toString() {
            return "TokenImpl {" +
                    "value = '" + value + '\'' +
                    " | tokenType = " + tokenType +
                    '}';
        }
    }
}