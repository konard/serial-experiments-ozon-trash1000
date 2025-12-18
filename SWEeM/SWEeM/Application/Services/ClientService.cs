using Microsoft.EntityFrameworkCore;
using SWEeM.Application.Contracts.Services;
using SWEeM.Application.Dtos;
using SWEeM.Application.Dtos.Client;
using SWEeM.Application.Mappers;
using SWEeM.Infrastructure.Persistence;

namespace SWEeM.Application.Services;

public class ClientService(AppDbContext dbContext) : IClientService
{
    public async Task<Guid> CreateAsync(CreateClientDto dto, CancellationToken cancellationToken = default)
    {
        var client = dto.ToClient();

        dbContext.Clients.Add(client);
        await dbContext.SaveChangesAsync(cancellationToken);
        return client.Id;
    }

    public async Task<PaginatedResult<ClientDto>> GetAllAsync(
        int page = 1,
        int pageSize = 10,
        CancellationToken cancellationToken = default)
    {
        var query = dbContext.Clients.AsNoTracking();
        var totalCount = await query.CountAsync(cancellationToken);

        var clients = await query
            .Skip((page - 1) * pageSize)
            .Take(pageSize)
            .ToListAsync(cancellationToken);

        var dtos = clients.Select(c => c.ToDto()!).ToList();
        return new PaginatedResult<ClientDto>(dtos, page, pageSize, totalCount);
    }

    public async Task<ClientDto?> GetByIdAsync(Guid id, CancellationToken cancellationToken = default)
    {
        var client = await dbContext.Clients.FindAsync(id, cancellationToken);
        return client?.ToDto();
    }

    public async Task<ClientDto?> UpdateAsync(Guid id, UpdateClientDto dto, CancellationToken cancellationToken = default)
    {
        var client = await dbContext.Clients.FindAsync(id, cancellationToken);

        if (client is null)
        {
            return null;
        }

        client.UpdateFrom(dto);

        await dbContext.SaveChangesAsync(cancellationToken);
        return client.ToDto();
    }

    public async Task<bool> DeleteAsync(Guid id, CancellationToken cancellationToken = default)
    {
        var client = await dbContext.Clients.FindAsync(id, cancellationToken);

        if (client is null)
        {
            return false;
        }

        dbContext.Clients.Remove(client);
        await dbContext.SaveChangesAsync(cancellationToken);
        return true;
    }
}