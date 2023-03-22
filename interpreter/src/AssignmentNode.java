public class AssignmentNode extends StatementNode {
    public final IdentifierNode identifier;
    public final ExpressionNode expression;

    public AssignmentNode(IdentifierNode identifier, ExpressionNode expression) {
        this.identifier = identifier;
        this.expression = expression;
    }
}
