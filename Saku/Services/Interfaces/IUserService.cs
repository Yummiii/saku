using Saku.ViewModels;

namespace Saku.Services.Interfaces;

public interface IUserService
{
    Task<UserViewModel> AddOrGetUser(UserRegisterViewModel filter);
}