package statements;

/**
 * Adds the value to container
 */
public class Adder extends Statement {

    private static final String STATEMENT = "add";

    public Adder(String VALUE) {
        super(STATEMENT, VALUE);
    }
}
