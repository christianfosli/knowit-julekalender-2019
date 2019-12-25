using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text.Json;
using System.Threading.Tasks;

namespace amazeing_race
{
    class Room
    {
        public int X {get; set;}
        public int Y {get; set;}
        public bool Nord { get; set; }
        public bool Vest { get; set; }
        public bool Syd { get; set; }
        public bool Aust { get; set; }
        public bool Visited {get; set;}
    }

    enum RobotType
    {
        Arthur,
        Isaac,
    }

    class Program
    {
        private const int Goal = 499;
        public List<List<Room>> Maze { get; private set; }
        public int VisitCount { get; private set; }

        private async Task LoadRooms()
        {
            using var fileStream = File.OpenRead("MAZE.TXT");
            var opt = new JsonSerializerOptions
            {
                PropertyNameCaseInsensitive = true
            };
            Maze = await JsonSerializer.DeserializeAsync<List<List<Room>>>(fileStream, opt);
        }

        private void Play(RobotType robotType)
        {
            var room = Maze[0][0];
            var visited = new Stack<Room>();
            while (room.X != Goal || room.Y != Goal)
            {
                if (!room.Visited) VisitCount++;
                room.Visited = true;
                visited.Push(room);
                IEnumerable<Room> movable = GetPotentialNextRooms(room);
                switch (robotType)
                {
                    case RobotType.Arthur:
                        room = movable
                            .OrderByDescending(r => r.Y)
                            .ThenByDescending(r => r.X)
                            .FirstOrDefault();
                        break;
                    case RobotType.Isaac:
                        room = movable.FirstOrDefault(r => r.X == room.X + 1) ?? // aust
                            movable.FirstOrDefault(r => r.Y == room.Y + 1) ??    // syd
                            movable.FirstOrDefault(r => r.X == room.X - 1) ??    // vest
                            movable.FirstOrDefault(r => r.Y == room.Y - 1);      // nord
                        break;

                }
                if (room is null) // Go back to the last room
                {
                    var thisOne = visited.Pop();
                    var lastOne = visited.Pop();
                    room = lastOne;
                }
            }

        }

        private IEnumerable<Room> GetPotentialNextRooms(Room room)
        {
            var movable = new List<Room>();
            if (!room.Syd) movable.Add(Maze[room.Y + 1][room.X]);
            if (!room.Aust) movable.Add(Maze[room.Y][room.X + 1]);
            if (!room.Vest) movable.Add(Maze[room.Y][room.X - 1]);
            if (!room.Nord) movable.Add(Maze[room.Y - 1][room.X]);
            return movable.Where(r => !r.Visited);
        }

        static void Main(string[] args)
        {
            var arthur = new Program();
            var isac = new Program();
            var arthurTask = Task.Run(async() =>
            {
                await arthur.LoadRooms();
                arthur.Play(RobotType.Arthur);
            });
            var isacTask = Task.Run(async () =>
            {
                await isac.LoadRooms();
                isac.Play(RobotType.Isaac);
            });
            arthurTask.Wait();
            isacTask.Wait();
            Console.WriteLine($"arthur visited {arthur.VisitCount} rooms, and isac visited {isac.VisitCount} rooms.");
            Console.WriteLine($"Difference: {Math.Abs(isac.VisitCount - arthur.VisitCount)}");
        }
    }
}
