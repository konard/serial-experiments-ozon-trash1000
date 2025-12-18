using SWEeM.Application.Dtos;
using SWEeM.Application.Dtos.Project;
using SWEeM.Domain.Entities;

namespace SWEeM.Application.Contracts.Services;

public interface IProjectService
{
    Task<Guid> CreateAsync(CreateProjectDto dto, CancellationToken ct = default);
    Task<PaginatedResult<ProjectDto>> GetAllAsync(int page = 1,
        int pageSize = 10,CancellationToken ct = default);
    Task<ProjectDto?> GetByIdAsync(Guid id, CancellationToken ct = default);
    Task<ProjectDto?> UpdateAsync(Guid id, UpdateProjectDto dto, CancellationToken ct = default);
    Task<bool> DeleteAsync(Guid id, CancellationToken ct = default);
}