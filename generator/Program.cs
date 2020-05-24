using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;

namespace generator
{
    [Flags]
    enum Mode
    {
	None = 0,
	Silent = 1<<0,
	Execute = 1<<1,
    }
    class Program
    {
	static string execstr=null;
	static string output = "file.h";
	static string[] files = null;
	static Mode ParseArgs(Taker<string> args)
	{
	    List<string> f = new List<string>();
	    Mode mode = Mode.None;

	    bool look=true;
	    while(args.TryTake(out var arg0))
	    {
		switch(arg0)
		{
		    case "-s" when look:
			mode |= Mode.Silent;
			break;
		    case "-e" when look:
			mode |= Mode.Execute;
			if(args.TryTake(out var exec))
			    execstr = exec;
			else
			    throw new InvalidOperationException("-e must be given a string");
			break;
		    case "-o" when look:
			if(!args.TryTake(out output))
			    throw new InvalidOperationException("-o must be given an output");
			break;
		    case "-" when look:
			look =false;
			break;
		    default:
			f.Add(arg0);
			break;
		}
	    }
	    files = f.ToArray();
	    return mode;
	}
        static void Main(string[] args)
        {
            Mode mode;
	    try {
		mode = ParseArgs(new Taker<string>(args));
	    } catch(Exception e)
	    {
		Console.WriteLine("Error: "+e.Message);
		return;
	    }

	    if(files.Length<1)
	    {
		Console.WriteLine("Usage: generator [-e <exec str>] [-s] [-] <files...>");
		return;
	    }
	    Console.WriteLine("Writing to "+output+"...");
	    using(var fs = new FileStream(output, FileMode.Create))
	    {
		using(var sw = new StreamWriter(fs)) {
		    if(mode.HasFlag(Mode.Silent))
			sw.WriteLine("#define SILENT");

		    sw.WriteLine($"constexpr const int DATA_COUNT = {files.Length};");
		    
		    if(mode.HasFlag(Mode.Execute)) {
			var exec_str = $"\"{CEscape(execstr)}\"";
			sw.WriteLine($"constexpr const char* const DATA_EXEC_AFTER = {exec_str};");
			sw.WriteLine($"static constexpr auto DATA_EXEC_AFTER_HASH = {exec_str}_sha256;");
		    }
		    else {
			sw.WriteLine("constexpr const char* const DATA_EXEC_AFTER = nullptr;");
			sw.WriteLine("static constexpr auto DATA_EXEC_AFTER_HASH = \"unbound\"_sha256;");
		    }

		    List<long> sizes= new List<long>();
		    sw.WriteLine("constexpr const unsigned char DATA[] = {");

		    foreach(var str in files)
		    {
			using(var ifs = new FileStream(str, FileMode.Open, FileAccess.Read))
			{
			    Console.Write(" + "+str);
			    try { 
			        sizes.Add(WriteFile(sw, ifs));
			        sw.WriteLine();
				Console.WriteLine(" OK");
			    } catch(Exception ex) {
				Console.WriteLine(" FAILED: "+ex.Message);
			    }
			}
		    }
		    sw.WriteLine("};");

		    Console.WriteLine("Adding lengths");
		    
		    sw.WriteLine("constexpr const long DATA_LENGTHS[DATA_COUNT] = {");
		    foreach(var len in sizes)
			sw.Write($"\t{len}ll,");
		    sw.WriteLine("\n};");

		    Console.WriteLine("Adding names");
		    
		    sw.WriteLine("constexpr const char* const DATA_NAMES[DATA_COUNT] = {");
		    foreach(var n in files)
			sw.Write("\t\""+CEscape(n.Split(Path.DirectorySeparatorChar)[^1])+"\",");
		    sw.WriteLine("};");
		}
	    }
        }

	static long WriteFile(StreamWriter to, Stream from)
	{
	    int rd;
	    long len=0;
	    while( (rd = from.ReadByte()) >= 0)
	    {
	        to.Write($"0x{((byte)rd):x2}, ");
		len+=1;
	    }
	    return len;
	}

	static string CEscape(string inp)
	{
	    return inp.Replace("\\", @"\\")
		.Replace("\"", "\\\"");
	}
    }
    class Taker<T>
    {
	readonly IEnumerator<T> iter;
	public Taker(IEnumerable<T> input)
	{
	    iter =input.GetEnumerator();
	}

	public bool TryTake(out T value)
	{
	    if(!iter.MoveNext()){
		value=default;
		return false;
	    }
	    value = iter.Current;
	    return true;
	}
    }
}
