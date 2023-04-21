package statements;

/**
 * Load a register
 */
public class Load extends Statement {

    private static final String STATEMENT = "load";

    public Load(String VALUE) {
        super(STATEMENT, VALUE);
    }
}
