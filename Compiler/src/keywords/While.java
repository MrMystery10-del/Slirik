package keywords;

import core.Lexer;
import statements.*;

import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;

public class While implements Keyword {

    private final List<Statement> body = new LinkedList<>();

    public While(List<Lexer.Token> tokens) {
        body.add(new Block("NONE"));

        Lexer.Token token;
        Iterator<Lexer.Token> iterator = tokens.iterator();

        while (iterator.hasNext()) {
            token = iterator.next();
            if (token.tokenType() == Lexer.TokenType.NUMBER || token.tokenType() == Lexer.TokenType.IDENTIFIER) {
                body.add(new Condition(token.value()));
            } else throw new IllegalArgumentException();

            if (iterator.hasNext())
                token = iterator.next();
            else break;

            if (token.tokenType() == Lexer.TokenType.CONDITION) {
                body.add(new ConditionOperation(token.value()));
            } else {
                //TODO implement multiple conditions
            }
        }

        body.add(new Skip());
    }

    @Override
    public List<Statement> getKeywordBody() {
        return body;
    }
}
