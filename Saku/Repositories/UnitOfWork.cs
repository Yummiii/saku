using Lina.DynamicServicesProvider.Attributes;
using Lina.UtilsExtensions;
using Microsoft.EntityFrameworkCore;
using MySqlConnector;
using Saku.Database;
using Saku.Exceptions;
using Saku.Repositories.Interfaces;

namespace Saku.Repositories;

[Repository(typeof(IUnitOfWork))]
public class UnitOfWork : IUnitOfWork
{
    private const int Duplicate = 1062;
    private readonly SakuDbContext _dbContext;

    public UnitOfWork(SakuDbContext dbContext)
    {
        _dbContext = dbContext;
    }

    public async Task SaveChanges()
    {
        try
        {
            await _dbContext.SaveChangesAsync();
        }
        catch (DbUpdateException ex)
        {
            var sqlException = ex.GetInnerException<MySqlException>();

            if (sqlException is null || sqlException.Number != Duplicate) throw;

            var message = sqlException.Message;
            throw new DuplicationException(message);
        }
    }
}