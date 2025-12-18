namespace SWEeM.Application.Dtos.Client;

public record UpdateClientDto(
    string Name,
    string Address,
    uint ProjectsTotal,
    uint  ProjectsCompleted);