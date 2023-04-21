package statements;

public abstract class Statement {

    private final String STATEMENT;
    private final String VALUE;

    protected Statement(String STATEMENT, String VALUE) {
        this.STATEMENT = STATEMENT;
        this.VALUE = VALUE;
    }

    /**
     * Generates a bytecode statement
     */
    @Override
    public String toString() {
        return STATEMENT + " " + VALUE;
    }
}
