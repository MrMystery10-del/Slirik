package keywords;

import core.Lexer;
import statements.*;

import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;

public class Else implements Keyword {

    private final List<Statement> body = new LinkedList<>();

    public Else() {
        body.add(new Block("NONE"));
    }

    @Override
    public List<Statement> getKeywordBody() {
        return body;
    }
}
