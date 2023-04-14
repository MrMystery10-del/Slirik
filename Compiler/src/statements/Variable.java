package statements;

/**
 * This statement creates a new variable
 */
public class Variable extends Statement {

    private static final String STATEMENT = "var";

    public Variable(String VALUE) {
        super(STATEMENT, VALUE);
    }
}
