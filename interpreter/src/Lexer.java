import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Lexer {

    interface Token {
        String value = null;
        TokenType tokenType = null;
    }

    enum TokenType {
        NUMBER, IDENTIFIER, EQUALS, OPEN_PAREN, CLOSE_PAREN, BINARY_OPERATOR, TYPE, END
    }

    public static Token[] tokenize(String s) {
        List<Token> tokenList = new ArrayList<>();
        int i = 0;
        while (i < s.length()) {
            char c = s.charAt(i);
            if (Character.isWhitespace(c)) {
                i++;
                continue;
            }
            if (Character.isDigit(c)) {
                StringBuilder numberBuilder = new StringBuilder();
                while (i < s.length() && Character.isDigit(s.charAt(i))) {
                    numberBuilder.append(s.charAt(i));
                    i++;
                }
                tokenList.add(new TokenImpl(numberBuilder.toString(), TokenType.NUMBER));
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
                    tokenList.add(new TokenImpl(identifier, TokenType.TYPE));
                } else {
                    tokenList.add(new TokenImpl(identifier, TokenType.IDENTIFIER));
                }
                continue;
            }
            if (c == '=') {
                tokenList.add(new TokenImpl("=", TokenType.EQUALS));
                i++;
                continue;
            }
            if (c == '(') {
                tokenList.add(new TokenImpl("(", TokenType.OPEN_PAREN));
                i++;
                continue;
            }
            if (c == ')') {
                tokenList.add(new TokenImpl(")", TokenType.CLOSE_PAREN));
                i++;
                continue;
            }
            if (c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '^' || c == '#') {
                tokenList.add(new TokenImpl(String.valueOf(c), TokenType.BINARY_OPERATOR));
                i++;
                continue;
            }
            if (c == ';') {
                tokenList.add(new TokenImpl(";", TokenType.END));
                break;
            }
            throw new IllegalArgumentException("Invalid character: " + c);
        }
        return tokenList.toArray(new Token[tokenList.size()]);
    }

    private record TokenImpl(String value, TokenType tokenType) implements Token {

        @Override
        public String toString() {
            return "TokenImpl{" +
                    "value='" + value + '\'' +
                    ", tokenType=" + tokenType +
                    '}';
        }
    }

    public static void main(String[] args) throws IOException {
        File file = new File("C:\\Users\\Mr.Mystery 1.0\\Desktop\\GitHub\\Slirik\\interpreter\\src\\Test.txt");
        BufferedReader reader = new BufferedReader(new FileReader(file));

        String s;
        while ((s = reader.readLine()) != null) {
            Token[] tokens = tokenize(s);
            Arrays.stream(tokens).forEach(t -> System.out.println(t));
        }

        reader.close();
    }
}