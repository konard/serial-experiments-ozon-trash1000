using SWEeM.Domain.Enums;

namespace SWEeM.Application.Dtos.User;

public record UpdateUserDto(
    string Name,
    string Login,
    string PasswordHash,
    Role Role);