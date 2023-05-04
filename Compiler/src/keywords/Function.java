package keywords;

import core.Lexer;
import statements.*;

import java.util.LinkedList;
import java.util.List;

public class Function implements Keyword {

    private final List<Statement> body = new LinkedList<>();

    public Function(List<Lexer.Token> tokens) {

    }

    @Override
    public List<Statement> getKeywordBody() {
        return body;
    }
}