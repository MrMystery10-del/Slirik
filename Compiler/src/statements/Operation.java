package statements;

/**
 * Sets the operation-type for next operation between numbers or containers
 */
public class Operation extends Statement {

    private static final String STATEMENT = "op";

    /**
     * @param VALUE
     *             + = PLUS,
     *             - = MINUS,
     *             * = MULTIPLY,
     *             / = DIVIDE,
     *             % = MODULO,
     *             ^ = X TO THE POWER OF Y,
     *             # = X ROOT OF Y
     */
    public Operation(String VALUE) {
        super(STATEMENT, VALUE);
    }
}
