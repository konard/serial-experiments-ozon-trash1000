namespace SWEeM.Domain.Entities;

public class Client
{
    public Guid Id { get; set; }
    public string Name { get; set; }
    public string Address { get; set; }
    public uint ProjectsTotal { get; set; }
    public uint ProjectsCompleted { get; set; }
}