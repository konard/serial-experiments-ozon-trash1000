using SWEeM.Application.Dtos;
using SWEeM.Application.Dtos.Client;
using SWEeM.Domain.Entities;

namespace SWEeM.Application.Contracts.Services;

public interface IClientService
{
    Task<Guid> CreateAsync(CreateClientDto dto, CancellationToken ct = default);
    Task<PaginatedResult<ClientDto>> GetAllAsync(int page = 1,
        int pageSize = 10,CancellationToken ct = default);
    Task<ClientDto?> GetByIdAsync(Guid id, CancellationToken ct = default);
    Task<ClientDto?> UpdateAsync(Guid id, UpdateClientDto dto, CancellationToken ct = default);
    Task<bool> DeleteAsync(Guid id, CancellationToken ct = default);
}