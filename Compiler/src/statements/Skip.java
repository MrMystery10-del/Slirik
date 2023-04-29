package statements;

/**
 * Skips until end statement when condition is false
 */
public class Skip extends Statement {

    private static final String STATEMENT = "skip";

    public Skip() {
        super(STATEMENT, "NONE");
    }
}
