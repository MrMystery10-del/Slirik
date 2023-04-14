import statements.Directory;
import statements.Getter;
import statements.Statement;
import statements.Variable;

import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Parser {

    private final List<Lexer.Token> tokens;

    private final Queue<Statement> statements = new LinkedList<>();
    private String currentDirectory = "global";
    private int index = 0;

    protected Parser(List<Lexer.Token> tokens) {
        this.tokens = tokens;
    }

    protected Queue<Statement> getStatements() {
        statements.add(new Directory(currentDirectory));

        while (index < tokens.size()) {
            switch (tokens.get(index).tokenType()) {
                case TYPE -> whenType();
                case IDENTIFIER -> whenIdentifier();
                case EQUALS -> whenEquals();
                case NUMBER -> whenNumber();
                case BINARY_OPERATOR -> whenBinaryOperation();
                case OPEN_PAREN -> whenOpenParen();
                case CLOSE_PAREN -> whenCloseParen();
                case END -> whenEnd();
            }
            index++;
        }
        return statements;
    }

    private void whenType() {
        if (index + 1 >= tokens.size())
            throw new IllegalArgumentException("No name for variable declared");
        index++;
        statements.add(new Variable(tokens.get(index).value()));
    }

    private void whenIdentifier() {

    }

    private void whenEquals() {
        if (index < 1)
            throw new IllegalArgumentException("Illegal start of statement");
        statements.add(new Getter(tokens.get(index - 1).value()));
    }

    private void whenNumber() {

    }

    private void whenBinaryOperation() {

    }

    private void whenOpenParen() {

    }

    private void whenCloseParen() {

    }

    private void whenEnd() {

    }
}
// int number = 20 + 10 * (20 - 30);

// dir global
// var number
// get number
// set 20
// operation plus
// add 10
// operation mult
// add 20
// operation minus
// add 30
