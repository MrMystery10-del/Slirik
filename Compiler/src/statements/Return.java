package statements;

/**
 * Returns a value to last caller
 */
public class Return extends Statement {

    private static final String STATEMENT = "return";

    public Return(String VALUE) {
        super(STATEMENT, VALUE);
    }
}
