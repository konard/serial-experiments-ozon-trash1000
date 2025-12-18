using SWEeM.Application.Dtos;
using SWEeM.Application.Dtos.Client;
using SWEeM.Application.Services;

public static class ClientEndpoints
{
    public static RouteGroupBuilder MapClientEndpoints(this WebApplication app)
    {
        var group = app.MapGroup("/clients")
            .WithTags("Clients")
            .WithDescription("Endpoints for managing clients and their information");

        group.MapGet("/", async (
            ClientService service,
            CancellationToken cancellationToken,
            int pageSize = 10,
            int page = 1) =>
        {
            var paginatedResult = await service.GetAllAsync(page, pageSize, cancellationToken);
            return Results.Ok(paginatedResult);
        })
        .WithName("GetAllClients")
        .WithSummary("Get all clients with pagination")
        .WithDescription("Retrieves a paginated list of all clients with their project statistics")
        .Produces<PaginatedResult<ClientDto>>(200, "application/json")
        .ProducesProblem(500);

        group.MapGet("/{id:guid}", async (Guid id, ClientService service, CancellationToken cancellationToken) =>
        {
            if (await service.GetByIdAsync(id, cancellationToken) is not { } client)
            {
                return Results.NotFound();
            }

            return Results.Ok(client);
        })
        .WithName("GetClientById")
        .WithSummary("Get a client by ID")
        .WithDescription("Retrieves a specific client by their unique identifier")
        .Produces<ClientDto>(200, "application/json")
        .Produces(404)
        .ProducesProblem(500);

        group.MapPost("/", async (CreateClientDto dto, ClientService service, CancellationToken cancellationToken) =>
        {
            try
            {
                var id = await service.CreateAsync(dto, cancellationToken);
                return TypedResults.Created($"/clients/{id}", id);
            }
            catch (ArgumentException ex)
            {
                return Results.BadRequest(ex.Message);
            }
        })
        .WithName("CreateClient")
        .WithSummary("Create a new client")
        .WithDescription("Creates a new client with the provided name and address")
        .Produces<Guid>(201, "application/json")
        .Produces(400)
        .ProducesProblem(500);

        group.MapPut("/{id:guid}", async (Guid id, UpdateClientDto dto, ClientService service, CancellationToken cancellationToken) =>
        {
            try
            {
                var client = await service.UpdateAsync(id, dto, cancellationToken);

                if (client is null)
                {
                    return Results.NotFound();
                }

                return Results.Ok(client);
            }
            catch (ArgumentException ex)
            {
                return Results.BadRequest(ex.Message);
            }
        })
        .WithName("UpdateClient")
        .WithSummary("Update an existing client")
        .WithDescription("Updates an existing client's information by their ID")
        .Produces<ClientDto>(200, "application/json")
        .Produces(404)
        .Produces(400)
        .ProducesProblem(500);

        group.MapDelete("/{id:guid}", async (Guid id, ClientService service, CancellationToken cancellationToken) =>
        {
            if (!await service.DeleteAsync(id, cancellationToken))
            {
                return Results.NotFound();
            }

            return Results.Ok(id);
        })
        .WithName("DeleteClient")
        .WithSummary("Delete a client")
        .WithDescription("Permanently deletes a client from the system by their ID")
        .Produces<Guid>(200, "application/json")
        .Produces(404)
        .ProducesProblem(500);

        return group;
    }
}
