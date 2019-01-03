using System;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace cs
{
    class Program
    {
        private static bool is_trap(bool[] prev_row, int i)
        {
            var l = i == 0 ? false : prev_row[i - 1];
            var r = i == prev_row.Length - 1 ? false : prev_row[i + 1];

            return l != r;
        }

        static void Main(string[] args)
        {
            var row1 = "......^.^^.....^^^^^^^^^...^.^..^^.^^^..^.^..^.^^^.^^^^..^^.^.^.....^^^^^..^..^^^..^^.^.^..^^..^^^..";
            //row1 = ".^^.^.^^^^";
            //int nrows = 40; // part 1

            int nrows = 400_000; // part 2
            //nrows = 10;

            int nlen = row1.Length;
            bool[] row = new bool[nlen];//row1.Select(x => x == '^').ToArray();
            int nsafe = 0;
            int i = 0;
            foreach (var n in row1)
            {
                row[i] = n == '^';
                if (!row[i])
                    nsafe += 1;
                i++;
            }

            var lists = new[] { row, new bool[nlen] };
            int lid = 0;
            int nlid = 0;

            for (int r = 0; r < nrows - 1; r++)
            {
                nlid = (lid + 1) % 2;
                row = lists[lid];
                var nextrow = lists[nlid];

                bool n = row[1] != false;
                //bool n = is_trap(row, 0);
                nextrow[0] = n;
                if (!n) nsafe++;

                n = row[nlen-2] != false;
                //n = is_trap(row, nlen - 1);
                nextrow[nlen-1] = n;
                if (!n) nsafe++;

                for (i = 1; i < nlen - 1; i++)
                {
                    n = row[i - 1] != row[i + 1];
                    //n = is_trap(row, i);
                    nextrow[i] = n;
                    if (!n)
                        nsafe += 1;
                }
                lid = nlid;
                /*
                if (r % 10000 == 0)
                    Console.WriteLine(r);
                    */
            }

            Console.WriteLine($"safe {nsafe}");
        }
    }
}
