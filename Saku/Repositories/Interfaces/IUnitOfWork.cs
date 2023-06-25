namespace Saku.Repositories.Interfaces;

public interface IUnitOfWork
{
    Task SaveChanges();
}