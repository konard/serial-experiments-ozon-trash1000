using SWEeM.Application.Dtos.User;
using SWEeM.Domain.Entities;

namespace SWEeM.Application.Mappers;

public static class UserMappers
{
    public static User ToUser(this CreateUserDto dto, string passwordHash)
        => new()
        {
            Id = Guid.NewGuid(),
            Name = dto.Name,
            Login = dto.Login,
            PasswordHash = passwordHash,
            Role = dto.Role
        };

    public static void UpdateFrom(this User user, UpdateUserDto dto, string? newPasswordHash = null)
    {
        user.Name = dto.Name;
        user.Login = dto.Login;
        user.Role = dto.Role;
        if (newPasswordHash is not null)
        {
            user.PasswordHash = newPasswordHash;
        }
    }

    public static UserDto? ToDto(this User? user)
    {
        if (user is null)
        {
            return null;
        }

        return new UserDto
        (
            user.Id,
            user.Name,
            user.Login,
            user.Role
        );
    }
}