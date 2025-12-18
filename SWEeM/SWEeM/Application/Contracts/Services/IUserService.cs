using SWEeM.Application.Dtos;
using SWEeM.Application.Dtos.User;
using SWEeM.Domain.Entities;

namespace SWEeM.Application.Contracts.Services;

public interface IUserService
{
    Task<Guid> CreateAsync(CreateUserDto dto, CancellationToken ct = default);
    Task<PaginatedResult<UserDto>> GetAllAsync(int page = 1,
        int pageSize = 10,CancellationToken ct = default);
    Task<UserDto?> GetByIdAsync(Guid id, CancellationToken ct = default);
    Task<UserDto?> UpdateAsync(Guid id, UpdateUserDto dto, CancellationToken ct = default);
    Task<bool> DeleteAsync(Guid id, CancellationToken ct = default);
}