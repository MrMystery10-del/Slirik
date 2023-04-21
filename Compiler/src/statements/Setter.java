package statements;

/**
 * Sets a value to current container
 */
public class Setter extends Statement {

    private static final String STATEMENT = "set";

    public Setter(String VALUE) {
        super(STATEMENT, VALUE);
    }
}
