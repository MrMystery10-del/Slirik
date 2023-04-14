package statements;

/**
 * This statement sets the current variable directory,
 * global directory or unique directory for each method
 */
public class Directory extends Statement {

    private static final String STATEMENT = "dir";

    public Directory(String VALUE) {
        super(STATEMENT, VALUE);
    }
}