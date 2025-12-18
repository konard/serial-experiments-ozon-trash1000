using SWEeM.Domain.Enums;

namespace SWEeM.Domain.Entities;

public class User
{
    public Guid Id { get; set; }
    public string Name { get; set; }
    public string Login { get; set; }
    public string PasswordHash { get; set; }
    public Role Role { get; set; }
}