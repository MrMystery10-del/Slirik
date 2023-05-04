package statements;

/**
 * Sets a required parameter for calling a method
 */
public class Parameter extends Statement {

    private static final String STATEMENT = "param";

    public Parameter(String VALUE) {
        super(STATEMENT, VALUE);
    }
}
