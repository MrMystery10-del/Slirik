package statements;

/**
 * Sets a block for back jumping
 */
public class Block extends Statement {

    private static final String STATEMENT = "block";

    public Block(String VALUE) {
        super(STATEMENT, VALUE);
    }
}
