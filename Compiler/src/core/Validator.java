package core;

import statements.Statement;

import java.util.HashSet;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Set;

public class Validator {

    private Queue<Statement> validated = new LinkedList<>(){
       @Override
        public boolean offer(Statement e) { return !this.contains(e) && super.add(e);}
    };

    private Queue<Statement> nonValidated;

    private Set<String> variables = new HashSet<>();
    private String directory = "";
    private String type = "";
    private String operator = "";

    protected Validator(Queue<Statement> statements) {
        this.nonValidated = statements;
    }

    protected Queue<Statement> getValidatedStatements() {
        while (!nonValidated.isEmpty()) {
            Statement statement = nonValidated.poll();

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

    private void validateDir(Statement statement, String value) {
        if (!directory.equals(value)) {
            validated.offer(statement);
            directory = value;
        }
    }

    private void validateOp(Statement statement, String value) {
        if (!operator.equals(value)) {
            validated.offer(statement);
            operator = value;
        }
    }

    private void validateType(Statement statement, String value) {
        if (!type.equals(value)) {
            validated.offer(statement);
            type = value;
        }
    }

    private void validateVar(Statement statement, String value) {
        if (variables.add(value)) {
            validated.offer(statement);
        }
    }
}
