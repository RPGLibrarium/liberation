package de.rpg_librarium.liberation;

import javax.persistence.EntityManager;
import javax.persistence.PersistenceContext;
import javax.transaction.Transactional;

import org.hibernate.Session;
import org.hibernate.SessionFactory;
import org.hibernate.Transaction;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/test")
public class SimpleREST {
	
	private SessionFactory sessionFactory;
	
	@RequestMapping(value = "/{name}", method = RequestMethod.GET)
	@Transactional
	public String getGreeting(@PathVariable String name) {
		String result = "Hello " + name;
		Session session = this.sessionFactory.openSession();
        Transaction tx = session.beginTransaction();
        Item item = new Item();
        session.persist(item);
        tx.commit();
        session.close();
		return result;
	}
}