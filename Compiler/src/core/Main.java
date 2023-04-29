package core;

import statements.Statement;

import java.awt.*;
import java.io.*;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Main {
    public static void main(String[] args) {
        long time;
        long compileTime;
        long writeTime;

        File inputFile;
        String outPutDirectory = "VirtualMachine/";

        if (args.length == 1) {
            inputFile = new File(args[0]);
        } else if (args.length == 2) {
            inputFile = new File(args[0]);
            outPutDirectory = args[1];
        } else inputFile = new File(selectFile());


        time = System.nanoTime();

        List<Lexer.Token> tokens = tokenizeFile(inputFile);

        Parser parser = new Parser(tokens);

        Queue<Statement> statements = parser.getStatements();
        compileTime = ((System.nanoTime() - time) / 1_000_000);
        time = System.nanoTime();

        generateBytecodeFile(outPutDirectory + inputFile.getName(), statements);
        writeTime = ((System.nanoTime() - time) / 1_000_000);

        System.out.println("Compile time: " + compileTime + "ms");
        System.out.println("Writing time: " + writeTime + "ms");
        System.out.println("Total time: " + (compileTime + writeTime) + "ms");

        System.exit(0);
    }

    // Returns a list of tokens which got generated based on provided file
    private static List<Lexer.Token> tokenizeFile(File file) {
        List<Lexer.Token> tokens = new LinkedList<>();

        try {
            BufferedReader reader = new BufferedReader(new FileReader(file));

            String s;
            while ((s = reader.readLine()) != null)
                tokens.addAll(Lexer.tokenize(s));
            reader.close();
        } catch (IOException exception) {
            throw new RuntimeException(exception);
        }

        return tokens;
    }

    private static void generateBytecodeFile(String fileIdentifier, Queue<Statement> statements) {
        File byteCode = new File(fileIdentifier.substring(0, fileIdentifier.length() - 3) + ".sks");
        try (FileWriter writer = new FileWriter(byteCode)) {
            while (!statements.isEmpty())
                writer.write(statements.poll().toString() + "\n");
        } catch (IOException exception) {
            exception.printStackTrace();
        }
    }

    private static String selectFile() {
        FileDialog dialog = new FileDialog(new Frame(), "Select .sk File");
        dialog.setMode(FileDialog.LOAD);
        dialog.setFilenameFilter((dir, name) -> name.endsWith(".sk"));
        dialog.setVisible(true);

        return dialog.getDirectory() + dialog.getFile();
    }
}