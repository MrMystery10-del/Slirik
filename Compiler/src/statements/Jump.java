package statements;

/**
 * Jump back to last block
 */
public class Jump extends Statement {

    private static final String STATEMENT = "jump";

    public Jump(String VALUE) {
        super(STATEMENT, VALUE);
    }
}