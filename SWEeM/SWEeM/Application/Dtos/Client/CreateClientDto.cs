namespace SWEeM.Application.Dtos.Client;

public record CreateClientDto(
    string Name,
    string Address,
    uint ProjectsTotal,
    uint  ProjectsCompleted);