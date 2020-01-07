using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace cs
{
    class Program
    {
        class Oper
        {
            public string Line { get; }
            public string Op { get; set; }
            public string Arg1 { get; }
            public int? IntArg1 { get; }
            public string Arg2 { get; }
            public int? IntArg2 { get; }

            public Oper(string[] args, string line)
            {
                Line = line;
                Op = args[0];
                Arg1 = args[1];
                if (int.TryParse(Arg1, out int i))
                    IntArg1 = i;
                if (args.Length > 2)
                {
                    Arg2 = args[2];
                    if (int.TryParse(Arg2, out int i2))
                        IntArg2 = i2;
                }
            }

            public override string ToString()
            {
                return $"({Op} {Arg1}/{IntArg1} {Arg2}/{IntArg2} {Line})";
            }
        }

        private static string regs2str<K, V>(IEnumerable<KeyValuePair<K, V>> regs) => string.Join(",", regs.Select(k => $"{k.Key}: {k.Value}"));

        private static bool runit(int a, List<Oper> program)
        {
            var regs = new Dictionary<string, int>()
            {
                ["a"] = a,
                ["b"] = 0,
                ["c"] = 0,
                ["d"] = 0
            };

            int ip = 0;
            int last_out = -1;
            int outs = 0;

            while (true)
            {
                if (ip >= program.Count || ip < 0)
                {
                    Console.WriteLine("HALT");
                    break;
                }

                var p = program[ip];

                if (p.Op == "cpy")
                {
                    if (!p.IntArg2.HasValue)
                        regs[p.Arg2] = p.IntArg1 ?? regs[p.Arg1];
                }
                else if (p.Op == "inc")
                    regs[p.Arg1] += 1;
                else if (p.Op == "dec")
                    regs[p.Arg1] -= 1;
                else if (p.Op == "jnz")
                {
                    if ((p.IntArg1.HasValue && p.IntArg1 != 0) || (!p.IntArg1.HasValue && regs[p.Arg1] != 0))
                        ip += (p.IntArg2 ?? regs[p.Arg2]) - 1;
                }
                else if (p.Op == "out")
                {
                    if (last_out == -1)
                        last_out = regs[p.Arg1];
                    else if (regs[p.Arg1] == last_out)
                        return false;
                    last_out = regs[p.Arg1];
                    outs += 1;
                }

                ip += 1;

                if (outs > 25)
                    return true;
            }

            return false;
        }

        static void Main(string[] args)
        {
            var program = new List<Oper>();
            foreach (var line in File.ReadAllLines("../25.txt"))
            {
                var m = line.Trim().Split(' ');
                program.Add(new Oper(m, line));
            }

            for (int i = 0; i < 9000; i++)
            {
                if (runit(i, program))
                {
                    Console.WriteLine($"found it {i}");
                    break;
                }
                else
                {
                    if (i % 1000 == 0)
                        Console.WriteLine($"i = {i}");
                }
            }
        }
    }
}
