package statements;

public abstract class Statement {

    private final String STATEMENT;
    private final String VALUE;

    public Statement(String STATEMENT, String VALUE) {
        this.STATEMENT = STATEMENT;
        this.VALUE = VALUE;
    }

    public String getStatement() {
        return STATEMENT;
    }

    public String getValue() {
        return VALUE;
    }

    @Override
    public String toString() {
        return STATEMENT + " " + VALUE;
    }
}
