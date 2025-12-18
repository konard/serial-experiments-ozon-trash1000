using SWEeM.Application.Dtos;
using SWEeM.Application.Dtos.User;
using SWEeM.Application.Services;

namespace SWEeM.API.Endpoints;

public static class UserEndpoints
{
    public static RouteGroupBuilder MapUserEndpoints(this WebApplication app)
    {
        var group = app.MapGroup("/users")
            .WithTags("Users")
            .WithDescription("Endpoints for managing users in the system");

        group.MapGet("/", async (
            UserService service,
            CancellationToken cancellationToken,
            int pageSize = 10,
            int page = 1) =>
        {
            var paginatedResult = await service.GetAllAsync(page, pageSize, cancellationToken);
            return Results.Ok(paginatedResult);
        })
        .WithName("GetAllUsers")
        .WithSummary("Get all users with pagination")
        .WithDescription("Retrieves a paginated list of all users in the system")
        .Produces<PaginatedResult<UserDto>>(200, "application/json")
        .ProducesProblem(500);

        group.MapGet("/{id:guid}", async (Guid id, UserService service, CancellationToken cancellationToken) =>
        {
            if (await service.GetByIdAsync(id, cancellationToken) is not { } user)
            {
                return Results.NotFound();
            }

            return Results.Ok(user);
        })
        .WithName("GetUserById")
        .WithSummary("Get a user by ID")
        .WithDescription("Retrieves a specific user by their unique identifier")
        .Produces<UserDto>(200, "application/json")
        .Produces(404)
        .ProducesProblem(500);

        group.MapPost("/", async (CreateUserDto dto, UserService service, CancellationToken cancellationToken) =>
        {
            try
            {
                var id = await service.CreateAsync(dto, cancellationToken);
                return TypedResults.Created($"/users/{id}", id);
            }
            catch (ArgumentException ex)
            {
                return Results.BadRequest(ex.Message);
            }
        })
        .WithName("CreateUser")
        .WithSummary("Create a new user")
        .WithDescription("Creates a new user with the provided details including name, login, password, and role")
        .Produces<Guid>(201, "application/json")
        .Produces(400)
        .ProducesProblem(500);

        group.MapPut("/{id:guid}", async (Guid id, UpdateUserDto dto, UserService service, CancellationToken cancellationToken) =>
        {
            try
            {
                var user = await service.UpdateAsync(id, dto, cancellationToken);

                if (user is null)
                {
                    return Results.NotFound();
                }

                return Results.Ok(user);
            }
            catch (ArgumentException ex)
            {
                return Results.BadRequest(ex.Message);
            }
        })
        .WithName("UpdateUser")
        .WithSummary("Update an existing user")
        .WithDescription("Updates an existing user's information by their ID")
        .Produces<UserDto>(200, "application/json")
        .Produces(404)
        .Produces(400)
        .ProducesProblem(500);

        group.MapDelete("/{id:guid}", async (Guid id, UserService service, CancellationToken cancellationToken) =>
        {
            if (!await service.DeleteAsync(id, cancellationToken))
            {
                return Results.NotFound();
            }

            return Results.Ok(id);
        })
        .WithName("DeleteUser")
        .WithSummary("Delete a user")
        .WithDescription("Permanently deletes a user from the system by their ID")
        .Produces<Guid>(200, "application/json")
        .Produces(404)
        .ProducesProblem(500);

        return group;
    }
}
