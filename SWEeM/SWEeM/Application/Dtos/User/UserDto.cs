using SWEeM.Domain.Enums;

namespace SWEeM.Application.Dtos.User;

public record UserDto(
    Guid Id,
    string Name,
    string Login,
    Role Role);