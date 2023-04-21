package statements;

/**
 * Sets an operation for comparing conditions
 */
public class ConditionOperation extends Statement {

    private static final String STATEMENT = "cop";

    public ConditionOperation(String VALUE) {
        super(STATEMENT, VALUE);
    }
}
