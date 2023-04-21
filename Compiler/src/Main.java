import statements.Statement;

import java.io.*;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Main {
    public static void main(String[] args) throws IOException {
        File file = new File("C:\\Users\\Mr.Mystery 1.0\\Desktop\\Slirik\\Example\\src\\Main.sk");
        BufferedReader reader = new BufferedReader(new FileReader(file));

        List<Lexer.Token> tokens = new LinkedList<>();

        String s;
        while ((s = reader.readLine()) != null)
            tokens.addAll(Lexer.tokenize(s));
        reader.close();

        Parser parser = new Parser(tokens);

        try {
            String name = file.getName();
            File byteCode = new File(name.substring(0, name.length() - 3) + ".sks");
            FileWriter writer = new FileWriter(byteCode);
            Queue<Statement> statements = parser.getStatements();
            while (!statements.isEmpty()) {
                writer.write(statements.poll().toString() + "\n");
            }
            writer.close();
        } catch (IOException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
    }
}