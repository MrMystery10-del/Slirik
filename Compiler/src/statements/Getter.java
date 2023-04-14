package statements;

/**
 * Gets the variable container
 */
public class Getter extends Statement {

    private static final String STATEMENT = "get";

    public Getter(String VALUE) {
        super(STATEMENT, VALUE);
    }
}
