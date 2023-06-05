package core;

import statements.Statement;

import java.util.HashSet;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Set;

public class Validator {
    // Queue to store validated statements
    private Queue<Statement> validated = new LinkedList<>();
    //Queue to store non-validated statements
    private Queue<Statement> nonValidated;

    // Set to store variables ecountered during the validation proccess
    private Set<String> variables = new HashSet<>();
    // Directory, type, and operator used for validation
    private String directory = "";
    private String type = "";
    private String operator = "";

    // Constructor for validator object with a queue of statements that need ot be validated
    protected Validator(Queue<Statement> statements) {
        this.nonValidated = statements;
    }

    // Returns the queue of validated statements
    protected Queue<Statement> getValidatedStatements() {
        while (!nonValidated.isEmpty()) {
            Statement statement = nonValidated.poll();
             // Checks to see the statement type
            switch (statement.getSTATEMENT()) {
                case "dir" -> validateDir(statement, statement.getVALUE());
                case "op" -> validateOp(statement, statement.getVALUE());
                case "type" -> validateType(statement, statement.getVALUE());
                case "var" -> validateVar(statement, statement.getVALUE());
                default -> validated.offer(statement);
            }
        }
        return validated;
    }

    //Validates the directory statement and updates the directory value as neccessary
    private void validateDir(Statement statement, String value) {
        if (!directory.equals(value)) {
            validated.offer(statement);
            directory = value;
        }
    }

    // Validates the operator statement and updates the operator value as neccesarry
    private void validateOp(Statement statement, String value) {
        if (!operator.equals(value)) {
            validated.offer(statement);
            operator = value;
        }
    }

    // Validates the type statememnt and updates the type value as neccessarry
    private void validateType(Statement statement, String value) {
        if (!type.equals(value)) {
            validated.offer(statement);
            type = value;
        }
    }

    // Validates the variable statement and adds it to the set of variables f it is unique
    private void validateVar(Statement statement, String value) {
        if (variables.add(value)) {
            validated.offer(statement);
        }
    }
}
