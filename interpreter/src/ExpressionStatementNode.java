public class ExpressionStatementNode extends StatementNode {
    public final ExpressionNode expression;

    public ExpressionStatementNode(ExpressionNode expression) {
        this.expression = expression;
    }
}
