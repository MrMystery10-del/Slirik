package statements;

/**
 * Sets the type for next variables
 */
public class Type extends Statement {

    private static final String STATEMENT = "type";

    public Type(String VALUE) {
        super(STATEMENT, VALUE);
    }
}