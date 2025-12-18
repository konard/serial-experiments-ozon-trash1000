using SWEeM.Application.Dtos.Client;
using SWEeM.Domain.Entities;

namespace SWEeM.Application.Mappers;

public static class ClientMappers
{
    public static Client ToClient(this CreateClientDto dto)
        => new()
        {
            Id = Guid.NewGuid(),
            Name = dto.Name,
            Address = dto.Address,
            ProjectsTotal = dto.ProjectsTotal,
            ProjectsCompleted = dto.ProjectsCompleted
        };

    public static void UpdateFrom(this Client client, UpdateClientDto dto)
    {
        client.Name = dto.Name;
        client.Address = dto.Address;
        client.ProjectsTotal = dto.ProjectsTotal;
        client.ProjectsCompleted = dto.ProjectsCompleted;
    }

    public static ClientDto? ToDto(this Client? client)
    {
        if (client is null)
        {
            return null;
        }

        return new ClientDto
        (
            client.Id,
            client.Name,
            client.Address,
            client.ProjectsTotal,
            client.ProjectsCompleted
        );
    }
}