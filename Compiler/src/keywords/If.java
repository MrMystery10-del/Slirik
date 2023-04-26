package keywords;

import core.Lexer;
import statements.*;

import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;

public class If implements Keyword {

    private final List<Statement> body = new LinkedList<>();

    public If(List<Lexer.Token> tokens) {
        Lexer.Token token;
        Iterator<Lexer.Token> iterator = tokens.iterator();

        while (iterator.hasNext()) {
            token = iterator.next();
            if (token.tokenType() == Lexer.TokenType.NUMBER) {
                body.add(new Condition(token.value()));

                if (iterator.hasNext())
                    token = iterator.next();
                else throw new IllegalArgumentException();

                if (token.tokenType() == Lexer.TokenType.CONDITION) {
                    body.add(new ConditionOperation(token.value()));
                } else {
                    //TODO implement multiple conditions
                }
            } else throw new IllegalArgumentException();
        }

        body.add(new Skip());
    }

    @Override
    public List<Statement> getKeywordBody() {
        return body;
    }
}
