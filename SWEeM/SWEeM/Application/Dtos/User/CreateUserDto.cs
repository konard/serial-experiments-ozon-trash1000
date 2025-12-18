using SWEeM.Domain.Enums;

namespace SWEeM.Application.Dtos.User;

public record CreateUserDto(
    string Name,
    string Login,
    string Password,
    Role Role);