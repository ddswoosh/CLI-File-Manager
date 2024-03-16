class Cli
{
    public void Run()
    {
        Console.Title = "Rust File Manager";
        Console.Write("Welcome to the Rust File Manager! Type -help at any time to display options for your current operation.\n");

        while (true)
        {
            Console.WriteLine("\n");
            Console.ForegroundColor = ConsoleColor.White;
            string cw = ("Enter a command: ");
            Console.WriteLine(cw);
            Console.ForegroundColor = ConsoleColor.Green;
            int promptLength = cw.Length;
            Console.SetCursorPosition(promptLength, Console.CursorTop-1);

            string userInput = Console.ReadLine();
        }
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