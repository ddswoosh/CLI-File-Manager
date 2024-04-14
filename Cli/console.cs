using System;
using System.IO;

class Cli
{
    public void Run()
    {
        Console.Title = "Rust File Manager";
        Console.Write("Welcome to the Rust File Manager! Type -help at any time to display options for your current operation.\n");

        string commandFilePath = @"C:\Users\ddswoosh\rust\dump\command.txt";
        string responseFilePath = @"C:\Users\ddswoosh\rust\dump\response.txt";

        while (true)
        {
            Console.WriteLine("\n");
            Console.ForegroundColor = ConsoleColor.White;

            string cw = ("Enter a command: ");

            Console.WriteLine(cw);
            Console.ForegroundColor = ConsoleColor.Green;

            int promptLength = cw.Length;

            Console.SetCursorPosition(promptLength, Console.CursorTop - 1);

            string userInput = Console.ReadLine();

            WriteCommandToFile(userInput, commandFilePath);
            Thread.Sleep(1000);

            string response = ReadResponseFromFile(responseFilePath);

            Console.WriteLine(response);
        }
    }

    private void WriteCommandToFile(string command, string filePath)
    {
        try
        {
            File.WriteAllText(filePath, command);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Could not write to file {ex.Message}");
        }
    }

    private string ReadResponseFromFile(string filePath)
    {
        return File.ReadAllText(filePath);

    }
}
class Run
{
    static void Main(string[] args)
    {
        Cli cli = new Cli();
        cli.Run();
    }
}
