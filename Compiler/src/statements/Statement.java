package statements;

import java.util.Objects;

public abstract class Statement {

    private final String STATEMENT;
    private final String VALUE;

    protected Statement(String STATEMENT, String VALUE) {
        this.STATEMENT = STATEMENT;
        this.VALUE = VALUE;
    }

    public String getSTATEMENT() {
        return STATEMENT;
    }

    public String getVALUE() {
        return VALUE;
    }

    /**
     * Generates a bytecode statement
     */
    @Override
    public String toString() {
        return STATEMENT + " " + VALUE;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Statement statement)) return false;
        return STATEMENT.equals(statement.STATEMENT) && VALUE.equals(statement.VALUE);
    }

    @Override
    public int hashCode() {
        return Objects.hash(STATEMENT, VALUE);
    }
}
