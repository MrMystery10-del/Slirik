package statements;

/**
 * Sets a value to check for condition
 */
public class Condition extends Statement {

    private static final String STATEMENT = "con";

    public Condition(String VALUE) {
        super(STATEMENT, VALUE);
    }
}